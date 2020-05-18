package cz.cvut.swa.bazaar.productservice.data

import com.fasterxml.jackson.annotation.JsonProperty
import java.math.BigDecimal
import java.time.LocalDateTime

data class ProductDTO(

        val title: String,
        val description: String,
        val price: BigDecimal,
        val sellerId: Long

)

data class ProductStatusUpdateDTO(

        val id: String,
        val newStatus: ProductStatus

)

data class ProductWithReviewsDTO (

        val id: String,
        val title: String,
        val description: String,
        val price: BigDecimal,
        val sellerId: Long,
        var status: ProductStatus,
        val postedDatetime: LocalDateTime,

        var reviews: List<ReviewDTO> = emptyList()

)

data class ReviewResponseDTO(
    val message: String? = null,
    val success: Boolean? = null,
    val data: List<ReviewDTO> = emptyList()
)

data class ReviewDTO(

        val id: Long,

        @JsonProperty("user_id")
        val userId: Long,

        val text: String,

        @JsonProperty("product_id")
        val productId: String,

        val created: Int,
        val rating: Int

)