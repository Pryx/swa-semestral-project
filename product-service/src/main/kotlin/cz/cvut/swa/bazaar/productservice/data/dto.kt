package cz.cvut.swa.bazaar.productservice.data

import java.math.BigDecimal

data class ProductDTO(

        val title: String,
        val description: String,
        val price: BigDecimal,
        val sellerId: String

)

data class ReviewDTO(

        val reviewerId: String,
        val comment: String,
        val rating: Float

)

data class ProductStatusUpdateDTO(

        val id: String,
        val newStatus: ProductStatus

)