package cz.cvut.swa.bazaar.productservice.service

import com.fasterxml.jackson.databind.ObjectMapper
import cz.cvut.swa.bazaar.productservice.BaseTest
import cz.cvut.swa.bazaar.productservice.data.EntityNotFoundException
import cz.cvut.swa.bazaar.productservice.data.NetworkException
import cz.cvut.swa.bazaar.productservice.data.Product
import cz.cvut.swa.bazaar.productservice.data.ProductRepository
import cz.cvut.swa.bazaar.productservice.fakeProduct
import cz.cvut.swa.bazaar.productservice.fakeReviewDTO
import cz.cvut.swa.bazaar.productservice.randomUuid
import org.junit.Assert
import org.junit.Test
import org.mockito.Mockito.`when`
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.mock.mockito.MockBean
import java.math.BigDecimal
import java.util.*

class ProductServiceTest : BaseTest() {

    @Autowired
    private lateinit var productService: ProductService

    @MockBean
    private lateinit var productRepository: ProductRepository

    @Autowired
    private lateinit var objectMapper: ObjectMapper

    @MockBean
    private lateinit var reviewService: ReviewService

    @Test(expected = EntityNotFoundException::class)
    fun `getProductWithReviews, product not found, should throw exception`() {
        val productId = randomUuid()

        `when`(productRepository.findById(productId)).thenReturn(Optional.empty())

        productService.getProductWithReviews(productId)
    }

    @Test(expected = NetworkException::class)
    fun `getProductWithReviews, product found, cannot fetch reviews, should throw exception`() {
        val productId = randomUuid()
        val product = Product(randomUuid(),
                "testProduct",
                "Cool product",
                BigDecimal.ONE,
                randomUuid())

        `when`(productRepository.findById(productId)).thenReturn(Optional.of(product))
        `when`(reviewService.fetchProductReviews(productId)).thenReturn(null)

        productService.getProductWithReviews(productId)
    }

    @Test
    fun `getProductWithReviews, product found, should fetch reviews, should return product`() {
        val productId = randomUuid()
        val product = fakeProduct()

        val reviewDTO = fakeReviewDTO()

        `when`(productRepository.findById(productId)).thenReturn(Optional.of(product))
        `when`(reviewService.fetchProductReviews(productId)).thenReturn(mutableListOf(reviewDTO))

        val productWithReviews = productService.getProductWithReviews(productId)

        Assert.assertNotNull(productWithReviews)
        Assert.assertFalse(productWithReviews.reviews.isEmpty())
    }

}